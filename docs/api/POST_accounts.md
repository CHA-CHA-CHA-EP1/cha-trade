# Account Service — API Documentation

## Overview

Account Service เป็น service หลักที่จัดการเรื่อง user account และ balance ทั้งหมด
ทุก service อื่น (Order, Matching) ต้องเรียกผ่าน Account Service เสมอเมื่อต้องการจัดการเงิน

---

## API Endpoints

### Public Endpoints

เส้นที่ client เรียกได้โดยตรง

| Method | Endpoint                | Description                                      |
| ------ | ----------------------- | ------------------------------------------------ |
| POST   | `/accounts`             | สร้าง account ใหม่พร้อม balance เริ่มต้น 100,000 |
| GET    | `/accounts/:id`         | ดู account info                                  |
| GET    | `/accounts/:id/balance` | ดู available และ hold balance                    |
| POST   | `/accounts/:id/deposit` | เติมเงินเข้า account                             |

---

### Internal Endpoints

เส้นที่ใช้เฉพาะ service อื่นเรียกเท่านั้น (Order Service, Matching Service)

| Method | Endpoint                         | Description                         |
| ------ | -------------------------------- | ----------------------------------- |
| POST   | `/internal/accounts/:id/hold`    | hold balance ก่อน place order       |
| POST   | `/internal/accounts/:id/release` | คืน hold balance เมื่อ cancel order |
| POST   | `/internal/accounts/:id/settle`  | ตัดเงินจริงเมื่อ order match สำเร็จ |

---

## Request / Response

### POST `/accounts`

สร้าง account ใหม่

**Response**

```json
{
  "id": "uuid",
  "available_balance": 100000,
  "hold_balance": 0,
  "created_at": "2024-01-01T00:00:00Z"
}
```

---

### GET `/accounts/:id/balance`

ดู balance ปัจจุบัน

**Response**

```json
{
  "account_id": "uuid",
  "available_balance": 70000,
  "hold_balance": 30000,
  "total_balance": 100000
}
```

---

### POST `/accounts/:id/deposit`

เติมเงิน

**Request**

```json
{
  "amount": 50000
}
```

**Response**

```json
{
  "account_id": "uuid",
  "available_balance": 150000,
  "hold_balance": 0
}
```

---

### POST `/internal/accounts/:id/hold`

Hold balance ก่อน place order

**Request**

```json
{
  "amount": 30000,
  "order_id": "uuid"
}
```

**Response**

```json
{
  "success": true,
  "available_balance": 70000,
  "hold_balance": 30000
}
```

> ⚠️ ต้องทำใน transaction + `FOR UPDATE` เสมอ เพื่อป้องกัน race condition

---

### POST `/internal/accounts/:id/release`

คืน hold balance เมื่อ cancel order

**Request**

```json
{
  "amount": 30000,
  "order_id": "uuid"
}
```

---

### POST `/internal/accounts/:id/settle`

ตัดเงินจริงเมื่อ order match สำเร็จ

**Request**

```json
{
  "from_account_id": "uuid",
  "to_account_id": "uuid",
  "amount": 30000,
  "order_id": "uuid"
}
```

> ⚠️ ต้องเป็น atomic transaction เท่านั้น — ตัดเงิน buyer และเพิ่มเงิน seller ต้องเกิดพร้อมกัน

---

## Balance Flow

```
deposit → available_balance เพิ่ม

place order → available_balance ลด / hold_balance เพิ่ม

cancel order → hold_balance ลด / available_balance เพิ่ม (คืนกลับ)

order match → hold_balance ลด (ตัดจริง) / seller available_balance เพิ่ม
```

---

## Database Schema

```sql
CREATE TABLE accounts (
  id                UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  available_balance NUMERIC(20, 8) NOT NULL DEFAULT 100000,
  hold_balance      NUMERIC(20, 8) NOT NULL DEFAULT 0,
  created_at        TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at        TIMESTAMP NOT NULL DEFAULT NOW()
);
```

---

## สรุป

| ประเภท             | จำนวนเส้น  |
| ------------------ | ---------- |
| Public endpoints   | 4 เส้น     |
| Internal endpoints | 3 เส้น     |
| **รวม**            | **7 เส้น** |
