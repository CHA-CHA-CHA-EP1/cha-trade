# Event: user.registered

## Overview

Published by auth service after a user is successfully created.
Consumed by account service to create a new account for the user.

---

## Topic

```
user.registered
```

## Producer

- **Service:** Auth Service
- **When:** After inserting a new row in the `users` table

## Consumer

- **Service:** Account Service
- **Action:** Create a new account for the user

---

## Message Payload

```json
{
  "user_id": "uuid",
  "occurred_at": "2024-01-01T00:00:00Z"
}
```

| Field        | Type      | Description                        |
| ------------ | --------- | ---------------------------------- |
| `user_id`    | UUID      | ID of the newly registered user    |
| `occurred_at`| Timestamp | Time the event was produced        |

---

## Consumer Business Logic

### Steps
1. Receive `user.registered` event from Kafka
2. Check that `user_id` does not already have an account → if exists, skip (idempotent)
3. Insert a new row in the `accounts` table with the given `user_id` and default balance of 100,000
4. Acknowledge the message

### Error Handling
- If insert fails → do not acknowledge → Kafka will redeliver the message
- Consumer must be **idempotent** — processing the same event twice must not create duplicate accounts (handled by step 2)
