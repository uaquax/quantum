# quantum

Telegram automation

# Example Config

```json
{
  "target_id": 111111111,,
  "back_messages": [
    {
      "reply": "Omg",
      "target": "yeah",
      "wait_message": null
    }
  ],
  "messages": [
    {
      "content": "Hi!",
      "wait_message": {
        "reply": "How are you?",
        "target": "hi",
        "wait_message": {
          "reply": "",
          "target": "@str",
          "wait_message": null
        }
      }
    },
    {
      "content": "So ok",
      "wait_message": {
        "reply": "Ohhh",
        "target": "@str",
        "wait_message": {
          "target": "stop",
          "reply": "Okay okay",
          "wait_message": {
            "target": "@str",
            "reply": "I understand. Sorry. Bye.",
            "wait_message": null
          }
        }
      }
    }
  ]
}
```
