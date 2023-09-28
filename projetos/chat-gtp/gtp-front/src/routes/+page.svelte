<title>Detector de comunistas</title>

<main>
    <header>
        <!-- get favicon as img -->
        <img src="/eagle.png" alt="favicon" width="64" height="64" />
        Detector de comunistas
    </header>
    <div class="chat-area">
        <div class="chat-item"></div>
    </div>
    <div class=chatbar>
        <input type="text" placeholder="Transcreva uma mensagem..." />
        <button><img src="/img.png" alt="Adicionar imagem" width=32></button>
        <button><img src="/send.png" alt="Enviar" width=32></button>
    </div>
</main>

<style>
@import url('https://fonts.googleapis.com/css2?family=Dela+Gothic+One&family=Work+Sans&display=swap');

main {
    background-color:rgb(192, 245, 236);
    width:100%;
    max-width:700px;
    margin:20px auto;
    border-radius: 20px;
    overflow: auto;
    padding-bottom: 20px;
}

header {
    font-family: 'Dela Gothic One', cursive;
    background-color: rgb(61, 39, 28);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size:1.8em;
    color:#fff;
    border-radius: 20px 20px 0 0;
    height: 96px;
}

header img {
    margin-right:10px;
}

.chat-area {
    background-color: #ebebeb;
    padding:20px;
    width:calc(100% - 80px);
    border-radius: 20px;
    margin: 20px auto 20px auto;
}

.chatbar {
    width:calc(100% - 40px);
    border-radius: 20px;
    height: 40px;
    background-color: white;
    margin: 0 auto;
    display: flex;
}

/* text input */
.chatbar input {
    border: none;
    outline: none;
    width:80%;
    height: 100%;
    padding: 0 20px;
    font-family: 'Work Sans', sans-serif;
    font-size: 1.2em;
    border-radius: 20px 0 0 20px;
}

.chatbar button {
    border: none;
    outline: none;
    width: 10%;
    height: 100%;
    background-color: rgb(104, 21, 21);
    color: #fff;
    font-family: 'Work Sans', sans-serif;
    font-size: 1.2em;
    display: flex;
    justify-content: center;
    align-items: center;
}

.chatbar button:last-of-type {
    border-radius: 0 20px 20px 0;
}

.chatbar button:hover {
    cursor: pointer;
    background-color: rgb(54, 15, 15)
}

</style>