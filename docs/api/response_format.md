# Standard Response Format

All API endpoints return the same JSON structure regardless of success or failure.

---

## Success

**HTTP Status:** `200`

```json
{
  "code": "0000",
  "message": "success",
  "data": { ... }
}
```

## Error

**HTTP Status:** non-200

```json
{
  "code": "XXXX",
  "message": "error message",
  "data": null
}
```

---

## Error Codes

| Code   | HTTP Status | Description                              |
| ------ | ----------- | ---------------------------------------- |
| `0000` | 200         | Success                                  |
| `4000` | 400         | Bad request / validation error           |
| `4004` | 404         | Resource not found                       |
| `4009` | 409         | Conflict (e.g. email or account already exists) |
| `4022` | 422         | Unprocessable (e.g. insufficient balance) |
| `5000` | 500         | Internal server error                    |

---

## Examples

### Success
```json
{
  "code": "0000",
  "message": "success",
  "data": {
    "user_id": "uuid",
    "message": "registration successful"
  }
}
```

### Not Found
```json
{
  "code": "4004",
  "message": "account not found",
  "data": null
}
```

### Conflict
```json
{
  "code": "4009",
  "message": "email already registered",
  "data": null
}
```

### Insufficient Balance
```json
{
  "code": "4022",
  "message": "insufficient balance",
  "data": null
}
```

### Internal Server Error
```json
{
  "code": "5000",
  "message": "internal server error",
  "data": null
}
```
