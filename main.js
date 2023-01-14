import init, { luhn, help_text } from "./pkg/luhney.js";

let messages, btn, input;

let history = [];
let yrotsih = [];

function sendMessage(msg) {
  input.value = "";
  addMessage("query", msg, "item-secondary");
  if (msg.startsWith("help")) {
    addMessage("luhney", help_text(), "item-primary");
  } else {
    let result = luhn(msg);
    addMessage("luhney", result ? "valid" : "invalid", "item-primary");
  }
}

function addMessage(from, msg, klass) {
  let formatted = `${from}: ${msg}`;
  console.log(formatted);
  let message = document.createElement("li");
  message.classList.add("message-item", klass);
  message.innerHTML = msg;
  messages.appendChild(message);
  messages.scrollTop = messages.scrollHeight;
}

function handleHistory(event) {
  input = document.getElementById("input");
  if (event.key == "ArrowUp") {
    let elem = history.pop();
    if (elem != undefined) {
      yrotsih.push(elem);
      input.value = elem;
    }
  } else if (event.key == "ArrowDown") {
    let elem = yrotsih.pop();
    if (elem != undefined) {
      history.push(elem);
      input.value = elem;
    } else {
      input.value = "";
    }
  }
}

document.addEventListener("DOMContentLoaded", () => {
  messages = document.querySelector(".message-list");
  btn = document.getElementById("btn");
  input = document.getElementById("input");
  input.focus();

  init().then(() => {
    btn.addEventListener("click", () => {
      let msg = input.value;
      sendMessage(msg);
      history.push(msg);
      input.focus();
    });
    window.addEventListener("keyup", function (e) {
      if (e.key == "Enter") {
        let msg = input.value;
        sendMessage(msg);
        history.push(msg);
      }
    });
    document.onkeydown = handleHistory;
    addMessage("luhney", help_text(), "item-primary");
  });
});
