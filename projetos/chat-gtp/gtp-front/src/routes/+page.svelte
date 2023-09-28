<script>
    let chatArea, chatInput, fileinput;

    // Função que manda adiciona um item ao chat e manda a mensagem para o servidor
    const send = async () => {
        const response = await fetch('http://127.0.0.1:3000/msg', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                msg: chatInput
            })
        });

        const chatItem = document.createElement('div');
        chatItem.classList.add('chat-item');
        chatItem.textContent = chatInput;
        chatArea.appendChild(chatItem);

        // if response == 1, add class comunista
        const responseJson = await response.json();
        if (responseJson.response == 1) {
            chatItem.classList.add('comunista');
        }

        // Clear input
        chatInput = '';
    }

    const onFileSelected = async (e) => {
        let image = e.target.files[0];
		let reader = new FileReader();
		reader.readAsDataURL(image);
		reader.onload = async (e) => {
			const imgStr = e.target.result;

            const imgItem = document.createElement('img');
            imgItem.src = imgStr;

            const response = await fetch('http://127.0.0.1:3000/msg', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    msg: imgStr
                })
            });

            const responseJson = await response.json();
            if (responseJson.response == 1) {
                imgItem.classList.add('comunista-img');
            }

            chatArea.appendChild(imgItem);
		};
    }
</script>

<title>Detector de comunistas</title>

<main>
    <header>
        <!-- get favicon as img -->
        <img src="/eagle.png" alt="favicon" width="64" height="64" />
        Detector de comunistas
    </header>
    <div class="chat-area" bind:this={chatArea}/>
    <div class=chatbar>
        <input type="text" placeholder="Transcreva uma mensagem..." bind:value={chatInput}/>
        <button on:click={()=>{fileinput.click();}}><img src="/img.png" alt="Adicionar imagem" width=32></button>
        <button on:click={send}><img src="/send.png" alt="Enviar" width=32></button>
        <input style="display:none" type="file" accept=".jpg, .jpeg, .png" on:change={onFileSelected} bind:this={fileinput} >
    </div>
</main>

<style>
@import url('https://fonts.googleapis.com/css2?family=Dela+Gothic+One&family=Work+Sans:wght@400;700&display=swap');

main {
    background-color:rgb(192, 245, 236);
    width:100%;
    max-width:700px;
    margin:20px auto;
    border-radius: 20px;
    overflow: auto;
    padding-bottom: 20px;
    font-family: 'Work Sans', sans-serif;
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

:global(.chat-item) {
    background-color: #fff;
    padding:20px;
    margin-bottom: 10px;
}

:global(.chat-item:last-of-type) {
    margin-bottom: 0;
}

:global(.comunista) {
    font-weight: 700;
    color: #ff0000;
}

:global(.comunista-img) {
    border: 4px solid #ff0000;
}

</style>