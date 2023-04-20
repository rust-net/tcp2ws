const base = 'http://127.0.0.1:6601/';

function api([uri]: TemplateStringsArray) {
    return base + uri;
}

export type Item = {
    name: string,
    desc: string,
    ws: string,
    listen: string,
}

export type Config = {
    port: number,
    item: Item[],
}

export default {
    config(): Promise<Config> {
        return fetch(api `api/config`).then(r => r.json())
    },
    start(item: Item) {
        return fetch(api `api/start`, {
            method: 'POST',
            body: JSON.stringify(item),
            headers: {
                'Content-Type': 'application/json',
            },
        }).then(async r => r.ok ? r.text() : Promise.reject(await r.text()))
    },
    stop(item: Item) {
        return fetch(api `api/stop`, {
            method: 'POST',
            body: JSON.stringify(item),
            headers: {
                'Content-Type': 'application/json',
            },
        }).then(async r => r.ok ? r.text() : Promise.reject(await r.text()))
    },
    list(): Promise<Item[]> {
        return fetch(api `api/list`).then(r => r.json())
    },
    exit() {
        return fetch(api `exit`).then(r => r.text())
    },
    compareItems(item1: Item, item2: Item): boolean {
        return (
            item1.name === item2.name &&
            item1.desc === item2.desc &&
            item1.ws === item2.ws &&
            item1.listen === item2.listen
        )
    },
}