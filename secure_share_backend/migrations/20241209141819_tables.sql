
-- Ensure the UUID extension is available for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,  
    public_key TEXT,            
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Files table
CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,  
    file_name VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    encrypted_aes_key BYTEA NOT NULL,  
    encrypted_file BYTEA NOT NULL,    
    iv BYTEA NOT NULL,                
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Shared links table (with required password and expiration date)
CREATE TABLE shared_links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    file_id UUID REFERENCES files(id) ON DELETE CASCADE, 
    recipient_user_id UUID REFERENCES users(id) ON DELETE CASCADE,  
    password VARCHAR(255) NOT NULL,  
    expiration_date TIMESTAMP WITH TIME ZONE NOT NULL,  
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
