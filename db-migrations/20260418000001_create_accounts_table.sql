-- Migration: Create users and accounts tables
-- Description: Users (auth/profile) and Accounts (balance tracking) are separate

-- ============================================
-- USERS TABLE - Auth and Profile
-- ============================================
CREATE TABLE IF NOT EXISTS users (
  id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  first_name       TEXT NOT NULL,        -- AES-GCM-256 encrypted (PII)
  first_name_hmac  TEXT NOT NULL,        -- HMAC-SHA256 for search
  last_name        TEXT NOT NULL,        -- AES-GCM-256 encrypted (PII)
  last_name_hmac   TEXT NOT NULL,        -- HMAC-SHA256 for search
  email            VARCHAR(255) NOT NULL UNIQUE,
  password_hash    TEXT NOT NULL,        -- argon2id hash
  created_at       TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at       TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Index for faster email lookups (login)
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- ============================================
-- ACCOUNTS TABLE - Balance Tracking
-- ============================================
CREATE TABLE IF NOT EXISTS accounts (
  id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id           UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  available_balance NUMERIC(20, 8) NOT NULL DEFAULT 100000,
  hold_balance      NUMERIC(20, 8) NOT NULL DEFAULT 0,
  created_at        TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at        TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Enforce 1 user = 1 account
CREATE UNIQUE INDEX IF NOT EXISTS idx_accounts_user_id_unique ON accounts(user_id);

-- Index for faster account lookups
CREATE INDEX IF NOT EXISTS idx_accounts_id ON accounts(id);

-- Add constraint: balance should never be negative
ALTER TABLE accounts
  ADD CONSTRAINT chk_available_balance_non_negative
  CHECK (available_balance >= 0);

ALTER TABLE accounts
  ADD CONSTRAINT chk_hold_balance_non_negative
  CHECK (hold_balance >= 0);

-- Trigger to auto-update updated_at column
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_users_updated_at
  BEFORE UPDATE ON users
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trigger_accounts_updated_at
  BEFORE UPDATE ON accounts
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

-- Comments
COMMENT ON TABLE users IS 'User authentication and profile information';
COMMENT ON COLUMN users.first_name IS 'AES-GCM-256 encrypted first name (PII)';
COMMENT ON COLUMN users.first_name_hmac IS 'HMAC-SHA256 of lowercase first name for search';
COMMENT ON COLUMN users.last_name IS 'AES-GCM-256 encrypted last name (PII)';
COMMENT ON COLUMN users.last_name_hmac IS 'HMAC-SHA256 of lowercase last name for search';
COMMENT ON COLUMN users.password_hash IS 'argon2id hashed password';

COMMENT ON TABLE accounts IS 'User accounts with available and hold balance tracking';
COMMENT ON COLUMN accounts.user_id IS 'FK to users table - one user has exactly one account';
COMMENT ON COLUMN accounts.available_balance IS 'Balance available for trading (not held)';
COMMENT ON COLUMN accounts.hold_balance IS 'Balance held for pending orders';

-- ============================================
-- TRANSACTIONS TABLE - Transaction Ledger (Audit)
-- ============================================
CREATE TYPE transaction_type AS ENUM (
  'deposit',
  'withdraw',
  'hold',
  'release',
  'settle'
);

CREATE TYPE transaction_direction AS ENUM (
  'credit',   -- balance เพิ่ม (เข้า)
  'debit'     -- balance ลด (ออก)
);

CREATE TABLE IF NOT EXISTS transactions (
  id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  account_id          UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  transaction_type    transaction_type NOT NULL,
  amount              NUMERIC(20, 8) NOT NULL,
  direction           transaction_direction NOT NULL,
  balance_before      NUMERIC(20, 8) NOT NULL,
  balance_after       NUMERIC(20, 8) NOT NULL,
  order_id            UUID NULL,
  trade_id            UUID NULL,
  reference           TEXT NULL,
  created_at          TIMESTAMP NOT NULL DEFAULT NOW(),
  -- Append-only: ไม่อนุญาตให้ delete/update
  CONSTRAINT transactions_readonly CHECK (true)
);

-- Indexes for audit queries
CREATE INDEX IF NOT EXISTS idx_transactions_account_id ON transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_transactions_created_at ON transactions(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_transactions_order_id ON transactions(order_id);
CREATE INDEX IF NOT EXISTS idx_transactions_trade_id ON transactions(trade_id);

-- Index for combined queries (ดู transaction history ของ account)
CREATE INDEX IF NOT EXISTS idx_transactions_account_created ON transactions(account_id, created_at DESC);

-- Constraint: amount ต้อง > 0
ALTER TABLE transactions
  ADD CONSTRAINT chk_amount_positive
  CHECK (amount > 0);

-- Comment
COMMENT ON TABLE transactions IS 'Transaction ledger - append-only log for all balance changes';
COMMENT ON COLUMN transactions.balance_before IS 'Balance state before this transaction';
COMMENT ON COLUMN transactions.balance_after IS 'Balance state after this transaction';
COMMENT ON COLUMN transactions.order_id IS 'Link to order if related to order operation';
COMMENT ON COLUMN transactions.trade_id IS 'Link to trade if related to matched trade';
COMMENT ON COLUMN transactions.reference IS 'External reference (bank ref, etc.)';
