const data = `data:;base64,{{ encrypted }}`

async function decrypt(nonce, cipherText) {
    const subtle = window.crypto.subtle || window.crypto.webkitSubtle
    if (!subtle) {
        throw Error('Your browser does not support Web Cryptography.')
    }
    const key = JSON.parse(sessionStorage.hashedPassword)
    const derivedKey = await subtle.importKey('jwk', key, 'AES-GCM', true, ['decrypt'])
    const decrypted = await subtle.decrypt({ name: 'AES-GCM', iv: nonce }, derivedKey, cipherText)
    const decoded = new TextDecoder().decode(decrypted)
    console.log(decoded)
    return decoded
}

let onload = () => {}
if (document.currentScript) {
    // Defer onload invocation until the script is loaded.
    const oldOnload = document.currentScript.onload
    document.currentScript.onload = function (ev) {
        onload = () => oldOnload.call(this, ev)
    }
}

fetch(data)
    .then(response => response.arrayBuffer())
    .then(encrypted => {
        const nonce = encrypted.slice(32, 44)
        const cipherText = encrypted.slice(44)
        return decrypt(nonce, cipherText)
    })
    .then(decoded => {
        console.log("Decrypted JS with saved password")
        eval?.(decoded)
        onload()
    })
    .catch(error => console.error(error))
