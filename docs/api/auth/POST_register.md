# POST /auth/register

## Curl

```bash
curl -X POST http://localhost:8081/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john@example.com",
    "password": "plaintext_password"
  }'
```

## Request Body

```json
{
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "password": "plaintext_password"
}
```

## Response Body

```json
{
  "user_id": "uuid",
  "message": "registration successful"
}
```

## Business Logic

### Validate Request Body

1. `first_name` must not be empty
2. `last_name` must not be empty
3. `email` must not be empty and must be a valid email format
4. `password` must not be empty and must be at least 8 characters

### Steps

1. Check that `email` is not already registered in the `users` table → if exists, return 409
2. Begin database transaction
3. Hash password with argon2id
4. Lowercase `first_name` and `last_name`
5. Encrypt `first_name` and `last_name` with AES-GCM-256
   - format: `hex(nonce[12] || ciphertext || tag[16])`
   - key: `AES_KEY` from env
6. Generate HMAC for `first_name` and `last_name`
   - `hmac_sha256(HMAC_KEY, lowercase(first_name))`
   - `hmac_sha256(HMAC_KEY, lowercase(last_name))`
   - key: `HMAC_KEY` from env (must be different from `AES_KEY`)
7. Insert a new row in the `users` table with encrypted values and HMAC values
8. Publish `user.registered` event to Kafka with `user_id` as payload
9. Commit transaction
10. Return `user_id` and success message

### Data stored in DB

| Field | Value |
|-------|-------|
| `first_name` | `hex(nonce || ciphertext || tag)` |
| `first_name_hmac` | `hmac_sha256(HMAC_KEY, "john")` |
| `last_name` | `hex(nonce || ciphertext || tag)` |
| `last_name_hmac` | `hmac_sha256(HMAC_KEY, "doe")` |
| `password_hash` | argon2id hash |
