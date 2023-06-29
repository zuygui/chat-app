let ws = new WebSocket("ws://localhost:8080/ws");

const messagesBox = document.getElementById("messages-box");

ws.addEventListener("open", (event) => {
  console.log("[open] Connection established");
});

ws.addEventListener("message", (event) => {
  console.log(`[message] Data received from server: %o`, event.data);

  const data = JSON.parse(event.data);

  const messageItem = document.createElement("li");
  messageItem.classList.add("message-item");

  const messageItemUsername = document.createElement("span");
  messageItemUsername.classList.add("message-item-username");
  messageItemUsername.innerText = data.username + ": ";
  messageItem.appendChild(messageItemUsername);

  const messageItemMessage = document.createElement("span");
  messageItemMessage.classList.add("message-item-message");
  messageItemMessage.innerText = data.message;
  messageItem.appendChild(messageItemMessage);

  messagesBox.appendChild(messageItem);
});

let t;
ws.addEventListener("close", (event) => {
  console.log(
    "[close] Connection closed cleanly,\n\tcode: %o\n\treason: %o",
    event.code,
    event.reason
  );

  // Try to reconnect in 5 seconds
  clearTimeout(t);
  t = setTimeout(() => {
    console.error("Failed to connect to server, retrying in 5 seconds...");
    try {
      ws = new WebSocket("ws://localhost:8080/ws");
      clearTimeout(t);
    } catch (error) {
      console.log(error);
    }
  }, 5000);
});

ws.addEventListener("error", (event) => {
  console.log("[error] %o", event);
});

// on message sent
const form = document.getElementById("message-form");
form.addEventListener("submit", (event) => {
  event.preventDefault();
  const username = document.getElementById("username-input").value;
  const msgInput = document.getElementById("message-input");
  const message = msgInput.value;
  msgInput.value = "";
  ws.send(JSON.stringify({ username, message }));

  const messageItem = document.createElement("li");
  messageItem.classList.add("message-item");

  const messageItemUsername = document.createElement("span");
  messageItemUsername.classList.add("message-item-username");
  messageItemUsername.innerText = "You: ";
  messageItem.appendChild(messageItemUsername);

  const messageItemMessage = document.createElement("span");
  messageItemMessage.classList.add("message-item-message");
  messageItemMessage.innerText = message;
  messageItem.appendChild(messageItemMessage);

  messagesBox.appendChild(messageItem);
});
