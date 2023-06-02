-- migrations/{}_seed_user.sql

INSERT INTO users (user_id, username, password_hash)
VALUES (
    'ddf8994f-d522-4659-8d02-c1d479057be6',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1'
    '$8gJuMx9bQ7I5+HkNbkG4jQ$N5hoQamabsUrsPZN2S0LxYD3WLnCmBuH4FNS8aZgICk'
);
