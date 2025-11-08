-- Add approval status enum
CREATE TYPE approval_status AS ENUM ('pending', 'approved', 'rejected');

-- Organizations table (already exists in initial schema, but adding missing columns)
ALTER TABLE organizations ADD COLUMN IF NOT EXISTS approval_status approval_status DEFAULT 'pending';
ALTER TABLE organizations ADD COLUMN IF NOT EXISTS approved_by UUID;
ALTER TABLE organizations ADD COLUMN IF NOT EXISTS approved_at TIMESTAMPTZ;

CREATE INDEX idx_organizations_approval ON organizations(approval_status, created_at DESC);
CREATE INDEX idx_organizations_email ON organizations(email);
