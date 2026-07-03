#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("GG4nDe5Smu2XmSdZiY6qhifjma95LG6H4Zvn7S54sGsY");

pub const DISCRIMINATOR_SIZE: usize = 8;
pub const NONCE_BYTES: usize = 24;
pub const MAX_CIPHERTEXT_BYTES: usize = 4 * 1024; // حجم متن 4 کیلو بایت باشه

#[program]
pub mod encrypted_messenger {
    use super::*;

    pub fn initialize_conversation(
        ctx: Context<InitializeConversation>,
        participant_a: Pubkey,
        participant_b: Pubkey,
    ) -> Result<()> {

        // برای جلوگیری از چت با خودش
        require_keys_neq!(
            participant_a,
            participant_b,
            MessagingError::SelfConversation
        );

        // مرتب سازی کلید ها برای یکتا بودن
        require!(
            participant_a.to_bytes() < participant_b.to_bytes(),
            MessagingError::ParticipantsOutOfOrder
        );

        // فقط افراد شرکت کننده واجد ارسال پیام هستند
        let payer_key = ctx.accounts.payer.key();
        require!(
            payer_key == participant_a || payer_key == participant_b,
            MessagingError::UnauthorizedParticipant
        );

        // ذخیره اطلاعات در حساب مکالمه
        let clock = Clock::get()?;
        let conversation = &mut ctx.accounts.conversation;
        conversation.participant_a = participant_a;
        conversation.participant_b = participant_b;
        conversation.next_message_id = 0;
        conversation.created_at = clock.unix_timestamp;
        conversation.last_message_at = 0;
        conversation.bump = ctx.bumps.conversation;

        emit!(ConversationInitialized {
            conversation: conversation.key(),
            participant_a,
            participant_b,
        });

        Ok(())
    }

    pub fn send_message(
        ctx: Context<SendMessage>,
        ciphertext: Vec<u8>, // متن رمز شده (از سمت کلاینت)
        nonce: [u8; NONCE_BYTES], // عدد تصادفی یک‌بار مصرف
    ) -> Result<()> {

        // اعتبارسنجی اندازه
        require!(!ciphertext.is_empty(), MessagingError::EmptyCiphertext);
        require!(
            ciphertext.len() <= MAX_CIPHERTEXT_BYTES,
            MessagingError::CiphertextTooLarge
        );

        let sender = ctx.accounts.sender.key();
        let conversation_key = ctx.accounts.conversation.key();
        let message_key = ctx.accounts.message.key();
        let data_key = ctx.accounts.message_data.key();
        let message_id = ctx.accounts.conversation.next_message_id;

        // بررسی مجوز: فرستنده باید عضو مکالمه باشد
        require!(
            ctx.accounts.conversation.has_participant(&sender),
            MessagingError::UnauthorizedParticipant
        );

        let recipient = ctx.accounts.conversation.other_participant(&sender)?;
        let sent_at = Clock::get()?.unix_timestamp;
        let ciphertext_len = ciphertext.len() as u32;

        //ذخیره متادیتا در حساب Message
        let message = &mut ctx.accounts.message;
        message.conversation = conversation_key;
        message.data_account = data_key;
        message.sender = sender;
        message.recipient = recipient;
        message.message_id = message_id;
        message.sent_at = sent_at;
        message.ciphertext_len = ciphertext_len;
        message.nonce = nonce;
        message.cipher_scheme = CipherScheme::XChaCha20Poly1305;
        message.bump = ctx.bumps.message;
        message.data_bump = ctx.bumps.message_data;

        // ذخیره متن رمز شده در حساب MessageData
        let message_data = &mut ctx.accounts.message_data;
        message_data.message = message_key;
        message_data.ciphertext = ciphertext;

        // به روز رسانی شمارنده پیام
        let conversation = &mut ctx.accounts.conversation;
        conversation.next_message_id = conversation
            .next_message_id
            .checked_add(1)
            .ok_or(MessagingError::MessageCounterOverflow)?;
        conversation.last_message_at = sent_at;

        emit!(MessageSent {
            conversation: conversation_key,
            message: message_key,
            message_data: data_key,
            sender,
            recipient,
            message_id,
            ciphertext_len,
        });

        Ok(())
    }
}

/*
1- ساخت حساب برای مکالمه
2- مشخص کردن پرداخت کننده
3- فضای مورد نیاز
4- عبارت منحصر به فرد مکالمه
*/
#[derive(Accounts)]
#[instruction(participant_a: Pubkey, participant_b: Pubkey)]
pub struct InitializeConversation<'info> {
    #[account(
        init,
        payer = payer,
        space = DISCRIMINATOR_SIZE + Conversation::INIT_SPACE,
        seeds = [
            Conversation::SEED,
            participant_a.as_ref(),
            participant_b.as_ref(),
        ],
        bump
    )]
    pub conversation: Account<'info, Conversation>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(ciphertext: Vec<u8>)]
pub struct SendMessage<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(
        mut,
        seeds = [
            Conversation::SEED,
            conversation.participant_a.as_ref(),
            conversation.participant_b.as_ref(),
        ],
        bump = conversation.bump
    )]
    pub conversation: Account<'info, Conversation>,

    //  ایجاد Message PDA
    // اینجا بیشتر متا دیتا پیام ذخیره میشه
    #[account(
        init,
        payer = sender,
        space = DISCRIMINATOR_SIZE + Message::INIT_SPACE,
        seeds = [
            Message::SEED,
            conversation.key().as_ref(),
            &conversation.next_message_id.to_le_bytes(),
        ],
        bump
    )]
    pub message: Account<'info, Message>,

    // ایجاد حساب اطلاعات پیام (اطلاعاتش کامل)
    #[account(
        init,
        payer = sender,
        space = MessageData::space(ciphertext.len()),
        seeds = [
            MessageData::SEED,
            message.key().as_ref(),
        ],
        bump
    )]
    pub message_data: Account<'info, MessageData>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Conversation {
    pub participant_a: Pubkey,
    pub participant_b: Pubkey,
    pub next_message_id: u64,
    pub created_at: i64,
    pub last_message_at: i64,
    pub bump: u8,
}

impl Conversation {
    pub const SEED: &'static [u8] = b"conversation";

    pub fn has_participant(&self, participant: &Pubkey) -> bool {
        self.participant_a == *participant || self.participant_b == *participant
    }

    pub fn other_participant(&self, participant: &Pubkey) -> Result<Pubkey> {
        if self.participant_a == *participant {
            Ok(self.participant_b)
        } else if self.participant_b == *participant {
            Ok(self.participant_a)
        } else {
            err!(MessagingError::UnauthorizedParticipant)
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct Message {
    pub conversation: Pubkey,       // ارجاع به مکالمه
    pub data_account: Pubkey,       // ارجاع به داده‌های رمز شده
    pub sender: Pubkey,             // فرستنده
    pub recipient: Pubkey,          // گیرنده
    pub message_id: u64,            // شماره پیام در مکالمه
    pub sent_at: i64,               // زمان ارسال
    pub ciphertext_len: u32,        // اندازه متن رمز شده
    pub nonce: [u8; NONCE_BYTES],   // عدد تصادفی
    pub cipher_scheme: CipherScheme, // الگوریتم رمزنگاری
    pub bump: u8,                   // bump برای PDA پیام
    pub data_bump: u8,              // bump برای PDA داده
}

impl Message {
    pub const SEED: &'static [u8] = b"message";
}

#[account]
pub struct MessageData {
    pub message: Pubkey,
    pub ciphertext: Vec<u8>,
}

impl MessageData {
    pub const SEED: &'static [u8] = b"message_data";

    pub fn space(ciphertext_len: usize) -> usize {
        DISCRIMINATOR_SIZE + 32 + 4 + ciphertext_len
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum CipherScheme {
    XChaCha20Poly1305,
}

#[event]
pub struct ConversationInitialized {
    pub conversation: Pubkey,
    pub participant_a: Pubkey,
    pub participant_b: Pubkey,
}

#[event]
pub struct MessageSent {
    pub conversation: Pubkey,
    pub message: Pubkey,
    pub message_data: Pubkey,
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub message_id: u64,
    pub ciphertext_len: u32,
}

#[error_code]
pub enum MessagingError {
    #[msg("A conversation cannot be opened with the same wallet twice.")]
    SelfConversation,
    #[msg("Conversation participants must be passed in lexicographic pubkey order.")]
    ParticipantsOutOfOrder,
    #[msg("Only a participant in the conversation can perform this action.")]
    UnauthorizedParticipant,
    #[msg("Encrypted messages must contain at least one ciphertext byte")]
    EmptyCiphertext,
    #[msg("Encrypted message payload is larger than the program limit.")]
    CiphertextTooLarge,
    #[msg("The message counter overflowed")]
    MessageCounterOverflow,
}
