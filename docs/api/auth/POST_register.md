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
3. Hash the password (bcrypt/argon2)
4. Encrypt `first_name` and `last_name` with AES-256
5. Insert a new row in the `users` table
6. Publish `user.registered` event to Kafka with `user_id` as payload
7. Commit transaction
8. Return `user_id` and success message
