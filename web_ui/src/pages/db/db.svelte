<script>
    import { onMount } from "svelte";

    let db;
    onMount(() => {
        var request = window.indexedDB.open("draft");
        request.onerror = function (event) {
            console.log("数据库打开报错");
        };
        request.onsuccess = function (event) {
            db = request.result;
            console.log("数据库打开成功");
        };
        request.onupgradeneeded = function () {
            db = request.result;
            if (!db.objectStoreNames.contains("persons")) {
                db.createObjectStore("persons", { keyPath: "id" });
            }
        };
    });
    const save = () => {
        let tranx = db.transaction("persons", "readwrite");
        let store = tranx.objectStore("persons");
        let request = store.add({ id: "123", name: "patrick", age: 4 });
        request.onsuccess = function () {
            // (4)
            console.log("person added to the store", request.result);
        };

        request.onerror = function () {
            console.log("Error", request.error);
        };
    };
    const query=()=>{
        let tranx = db.transaction("persons", "readwrite");
        let store = tranx.objectStore("persons");
        const request=store.getAll()
        request.onsuccess=function(){
            console.log(request.result)
        }
    }
    const deleteSingle=()=>{
        let tranx = db.transaction("persons", "readwrite");
        let store = tranx.objectStore("persons");
        const request=store.delete("123")
        request.onsuccess=function(){
            console.log(request.result)
        }
    }
    const updateSingle=()=>{
        let tranx = db.transaction("persons", "readwrite");
        let store = tranx.objectStore("persons");
        const request=store.put({id:"123",name:"penny",age:28});
        request.onsuccess=function(){
            console.log(request.result)
        }
    }
</script>

<div>
    <button on:click={save}>save</button>
    <button on:click={query}>query</button>
    <button on:click={deleteSingle}>delete</button>
    <button on:click={updateSingle}>update</button>
</div>
