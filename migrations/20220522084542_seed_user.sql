-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '9b7711b9-52d0-43d0-8e8a-8ce73b6a9c87',
    'admin',
    '$argon2id$v=19$m=1500,t=2,p=1$rYM8VfRl1J3yChDx6m3IFg$8F4jhpZpJeKdaB9RFY8XIRHHgMDPHwNOEeZXPQBXR5M'
)
