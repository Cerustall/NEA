const ws = new WebSocket("/ws");
ws.onopen = () => {
    const username = (Math.random() + 1).toString(36).substring(7); // stackoverflow somewhere yeahh
    ws.send(username);
};

ws.onmessage = (event) => {
    const el = document.createElement("p");
    el.innerText = event.data;
    document.body.appendChild(el);
}
document.getElementById("send").onclick = () => {
    const text = document.getElementById("input").value;
    ws.send(text);
}