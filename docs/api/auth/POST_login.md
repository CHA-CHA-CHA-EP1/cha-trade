# POST /auth/login

## Curl

```bash
curl -X POST http://localhost:8081/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "plaintext_password"
  }'
```

## Request Body

```json
{
  "email": "john@example.com",
  "password": "plaintext_password"
}
```

## Response Body

```json
{
  "status": "success",
  "data": {
    "access_token": "jwt_token"
  }
}
```

## Business Logic

### Validate Request Body
1. `email` must not be empty and must be a valid email format
2. `password` must not be empty

### Steps
1. Find user by `email` in the `users` table → if not found, return 401
2. Verify password against `password_hash` using argon2id → if invalid, return 401
3. Generate JWT token with `user_id` as subject (`sub`)
   - Expiry: 24 hours from now
   - Secret: `JWT_SECRET` from env
4. Return `access_token`
