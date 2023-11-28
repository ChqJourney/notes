<script>
  import { onMount } from "svelte";
  import { Editor, isActive } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import BubbleMenu from "@tiptap/extension-bubble-menu";
  import FloatingMenu from "@tiptap/extension-floating-menu";
  import Bubble from "./Bubble.svelte";
  import Float from "./Float.svelte";
  import Image from "@tiptap/extension-image";
  import { editorTickStore, reset, tick } from "./editorStore";
  
  export let id="",title="", htmlConent="",textContent="";
  let editor;
  let element;
  let a_token;
  let interval;
  let to_be_saved_Note;
  const setImage=(url)=>editor.commands.setImage({src:url});
  onMount(() => {
    to_be_saved_Note={id:id,title:"",htmlConent:htmlConent,textContent:textContent}
    editor = new Editor({
      element: element,
      extensions: [
        StarterKit,
        Image,
        BubbleMenu.configure({
          element: document.querySelector("#bubble-menu"),
        }),
        FloatingMenu.configure({
          element: document.querySelector("#floating-menu"),
        }),
      ],
      content: htmlConent,
      onCreate({editor}){
        if(id===""){
          setTimeout(async()=>{
            let res=await fetch("https://www.photonee.com/api/biz/note",{
              method:"POST",
              body:JSON.stringify({
                title:title,
                html_content:editor.getHTML(),
                text_content:editor.getText(),
              })
            })
            let return_id=await res.json()
            to_be_saved_Note.id=return_id;
          },1000)

        }
      },
      onUpdate({ editor }) {
        
        reset();
        clearInterval(interval);
        interval = setInterval(() => {
          to_be_saved_Note.html_content=editor.getHTML()
          to_be_saved_Note.text_content=editor.getText()
            tick(async() => {
              try{
                let res=await fetch("https://www.photonee.com/api/biz/note",{
                  method:"PUT",
                  body:JSON.stringify(to_be_saved_Note)
                })
              }catch(ex){
                console.log(ex)
              }
              // console.log(await res.json())
              clearInterval(interval);
            });
          
          }, 1000);
   
      },
      onDestroy({}) {
        clearInterval(interval);
      },
      editorProps: {
        handleDrop: function (view, event, slice, moved) {
          if (
            !moved &&
            event.dataTransfer &&
            event.dataTransfer.files &&
            event.dataTransfer.files[0]
          ) {
            // if dropping external files
            // handle the image upload
            let formData = new FormData();
            formData.append("filej.png", event.dataTransfer.files[0]);
            fetch("http://www.photonee.com/api/file/upload", {
              method: "post",
              headers: {
                "Content-Type": "multipart/form-data",
                Authorization: `Bearer ${a_token}`,
              },
              body: formData,
            })
              .then((data) => {
                data.json().then(res=>{
                  console.log(res)
                  setImage(res.path)
                });
                //todo: insert pic url to content
              })
              .catch((error) => {
                console.log(error);
              });
            // console.log(event.dataTransfer.files[0]);
            return true; // handled
          }
          return false;
        },

        attributes: {
          class:
            "prose w-full h-[800px] px-4 py-1 dark:prose-invert prose-sm sm:prose-base lg:prose-lg border-0 xl:prose-2xl focus:outline-none",
        },
      },
    });

    return () => {
      editor.destroy();
    };
  });
  let currentEditStatus = [];

  const clickFire = (type) => {
    switch (type) {
      case "strong":
        editor.chain().focus().toggleBold().run();
        break;
      case "i":
        editor.chain().focus().toggleItalic().run();
        break;
      case "s":
        editor.chain().focus().toggleStrike().run();
        break;
      case "u":
        editor.chain().focus().toggleUnderline().run();
        break;
      default:
        break;
    }
    if (currentEditStatus.includes(type)) {
      currentEditStatus = currentEditStatus.filter((f) => f !== type);
    } else {
      currentEditStatus.push(type);
      currentEditStatus = currentEditStatus;
    }
  };
</script>

<div class="w-full h-[800px]">
  <input on:change={e=>to_be_saved_Note.title=e.target.value}
    type="text"
    id="large-input"
    class="block w-full p-4 text-gray-900 border-0 bg-transparent text-2xl font-semibold focus:outline-none focus:ring-0 dark:text-white"
  />

  <hr />

  <div spellcheck="false" bind:this={element} />
  {#if editor}
    <Bubble {editor}>
      <div class="bg-gray-500 rounded-sm">
        <button
          on:click={() => clickFire("strong")}
          class={`border-0  p-1 ${
            currentEditStatus.includes("strong")
              ? "bg-gray-400 fill-white hover:bg-gray-500"
              : "hover:bg-gray-400 fill-sky-400 hover:fill-white "
          } rounded-md`}
        >
          <svg
            class="h-5 w-5"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            width="200"
            height="200"
            ><path
              d="M533.333333 170.666667h-213.333333A64 64 0 0 0 256 234.666667v554.666666A64 64 0 0 0 320 853.333333h213.333333c138.154667 0 234.666667-87.722667 234.666667-213.333333 0-66.773333-33.237333-122.112-81.152-159.658667A200.021333 200.021333 0 0 0 725.333333 362.666667C725.333333 269.568 658.048 170.666667 533.333333 170.666667zM384 298.666667h149.333333c50.517333 0 64 40.234667 64 64s-13.482667 64-64 64H384V298.666667z m149.333333 426.666666H384v-170.666666h149.333333c50.346667 0 106.666667 36.48 106.666667 85.333333 0 53.418667-39.893333 85.333333-106.666667 85.333333z"
              fill=""
            /></svg
          >
        </button>
        <button
          on:click={() => clickFire("i")}
          class={`border-0  p-1 ${
            currentEditStatus.includes("i")
              ? "bg-gray-400 fill-white hover:bg-gray-500"
              : "hover:bg-gray-400 fill-sky-400 hover:fill-white "
          } rounded-md`}
        >
          <svg
            class="h-5 w-5"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            width="200"
            height="200"
            ><path
              d="M704 170.666667l2.133333 32c-32 6.4-53.333333 14.933333-66.133333 25.6-12.8 10.666667-19.2 23.466667-23.466667 38.4l-74.666666 477.866666c-4.266667 19.2 0 32 6.4 38.4 8.533333 8.533333 25.6 12.8 55.466666 14.933334v55.466666H320v-55.466666c70.4-6.4 108.8-25.6 113.066667-55.466667l98.133333-473.6c2.133333-6.4 2.133333-12.8 2.133333-17.066667v-12.8c0-4.266667-2.133333-6.4-4.266666-10.666666-2.133333-2.133333-6.4-6.4-8.533334-6.4-2.133333-2.133333-6.4-4.266667-12.8-6.4-6.4-2.133333-10.666667-4.266667-14.933333-4.266667-4.266667-2.133333-10.666667-2.133333-19.2-6.4-4.266667-2.133333-8.533333-2.133333-10.666667-2.133333v-32h241.066667z"
            /></svg
          >
        </button>
        <button
          on:click={() => clickFire("s")}
          class={`border-0  p-1 ${
            currentEditStatus.includes("s")
              ? "bg-gray-400 fill-white hover:bg-gray-500"
              : "hover:bg-gray-400 fill-sky-400 hover:fill-white "
          } rounded-md`}
        >
          <svg
            class="h-5 w-5"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            width="200"
            height="200"
            ><path
              d="M952 474H569.9c-10-2-20.5-4-31.6-6-15.9-2.9-22.2-4.1-30.8-5.8-51.3-10-82.2-20-106.8-34.2-35.1-20.5-52.2-48.3-52.2-85.1 0-37 15.2-67.7 44-89 28.4-21 68.8-32.1 116.8-32.1 54.8 0 97.1 14.4 125.8 42.8 14.6 14.4 25.3 32.1 31.8 52.6 1.3 4.1 2.8 10 4.3 17.8 0.9 4.8 5.2 8.2 9.9 8.2h72.8c5.6 0 10.1-4.6 10.1-10.1v-1c-0.7-6.8-1.3-12.1-2-16-7.3-43.5-28-81.7-59.7-110.3-44.4-40.5-109.7-61.8-188.7-61.8-72.3 0-137.4 18.1-183.3 50.9-25.6 18.4-45.4 41.2-58.6 67.7-13.5 27.1-20.3 58.4-20.3 92.9 0 29.5 5.7 54.5 17.3 76.5 8.3 15.7 19.6 29.5 34.1 42H72c-4.4 0-8 3.6-8 8v60c0 4.4 3.6 8 8 8h433.2c2.1 0.4 3.9 0.8 5.9 1.2 30.9 6.2 49.5 10.4 66.6 15.2 23 6.5 40.6 13.3 55.2 21.5 35.8 20.2 53.3 49.2 53.3 89 0 35.3-15.5 66.8-43.6 88.8-30.5 23.9-75.6 36.4-130.5 36.4-43.7 0-80.7-8.5-110.2-25-29.1-16.3-49.1-39.8-59.7-69.5-0.8-2.2-1.7-5.2-2.7-9-1.2-4.4-5.3-7.5-9.7-7.5h-79.7c-5.6 0-10.1 4.6-10.1 10.1v1c0.2 2.3 0.4 4.2 0.6 5.7 6.5 48.8 30.3 88.8 70.7 118.8 47.1 34.8 113.4 53.2 191.8 53.2 84.2 0 154.8-19.8 204.2-57.3 25-18.9 44.2-42.2 57.1-69 13-27.1 19.7-57.9 19.7-91.5 0-31.8-5.8-58.4-17.8-81.4-5.8-11.2-13.1-21.5-21.8-30.8H952c4.4 0 8-3.6 8-8v-60c0-4.3-3.6-7.9-8-7.9z"
            /></svg
          >
        </button>
        <button
          on:click={() => clickFire("u")}
          class={`border-0  p-1 ${
            currentEditStatus.includes("u")
              ? "bg-gray-400 fill-white hover:bg-gray-500"
              : "hover:bg-gray-400 fill-sky-400 hover:fill-white "
          } rounded-md`}
        >
          <svg
            class="h-5 w-5"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            width="200"
            height="200"
            ><path
              d="M341.333333 128v384a170.666667 170.666667 0 1 0 341.333334 0V128h85.333333v384a256 256 0 1 1-512 0V128h85.333333zM170.666667 853.333333h682.666666v85.333334H170.666667v-85.333334z"
            /></svg
          >
        </button>
      </div>
    </Bubble>
    <Float {editor}>
      <div class="bg-gray-500 rounded-sm">
        <button
          on:click={() => clickFire("strong")}
          class={`border-0  p-1 ${
            currentEditStatus.includes("strong")
              ? "bg-gray-400 fill-white hover:bg-gray-500"
              : "hover:bg-gray-400 fill-sky-400 hover:fill-white "
          } rounded-md`}
        >
          <svg
            class="h-5 w-5"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            width="200"
            height="200"
            ><path
              d="M533.333333 170.666667h-213.333333A64 64 0 0 0 256 234.666667v554.666666A64 64 0 0 0 320 853.333333h213.333333c138.154667 0 234.666667-87.722667 234.666667-213.333333 0-66.773333-33.237333-122.112-81.152-159.658667A200.021333 200.021333 0 0 0 725.333333 362.666667C725.333333 269.568 658.048 170.666667 533.333333 170.666667zM384 298.666667h149.333333c50.517333 0 64 40.234667 64 64s-13.482667 64-64 64H384V298.666667z m149.333333 426.666666H384v-170.666666h149.333333c50.346667 0 106.666667 36.48 106.666667 85.333333 0 53.418667-39.893333 85.333333-106.666667 85.333333z"
              fill=""
            /></svg
          >
        </button>
        <button
          on:click={() => clickFire("i")}
          class={`border-0  p-1 ${
            currentEditStatus.includes("i")
              ? "bg-gray-400 fill-white hover:bg-gray-500"
              : "hover:bg-gray-400 fill-sky-400 hover:fill-white "
          } rounded-md`}
        >
          <svg
            class="h-5 w-5"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            width="200"
            height="200"
            ><path
              d="M704 170.666667l2.133333 32c-32 6.4-53.333333 14.933333-66.133333 25.6-12.8 10.666667-19.2 23.466667-23.466667 38.4l-74.666666 477.866666c-4.266667 19.2 0 32 6.4 38.4 8.533333 8.533333 25.6 12.8 55.466666 14.933334v55.466666H320v-55.466666c70.4-6.4 108.8-25.6 113.066667-55.466667l98.133333-473.6c2.133333-6.4 2.133333-12.8 2.133333-17.066667v-12.8c0-4.266667-2.133333-6.4-4.266666-10.666666-2.133333-2.133333-6.4-6.4-8.533334-6.4-2.133333-2.133333-6.4-4.266667-12.8-6.4-6.4-2.133333-10.666667-4.266667-14.933333-4.266667-4.266667-2.133333-10.666667-2.133333-19.2-6.4-4.266667-2.133333-8.533333-2.133333-10.666667-2.133333v-32h241.066667z"
            /></svg
          >
        </button>
        <button
          on:click={() => clickFire("s")}
          class={`border-0  p-1 ${
            currentEditStatus.includes("s")
              ? "bg-gray-400 fill-white hover:bg-gray-500"
              : "hover:bg-gray-400 fill-sky-400 hover:fill-white "
          } rounded-md`}
        >
          <svg
            class="h-5 w-5"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            width="200"
            height="200"
            ><path
              d="M952 474H569.9c-10-2-20.5-4-31.6-6-15.9-2.9-22.2-4.1-30.8-5.8-51.3-10-82.2-20-106.8-34.2-35.1-20.5-52.2-48.3-52.2-85.1 0-37 15.2-67.7 44-89 28.4-21 68.8-32.1 116.8-32.1 54.8 0 97.1 14.4 125.8 42.8 14.6 14.4 25.3 32.1 31.8 52.6 1.3 4.1 2.8 10 4.3 17.8 0.9 4.8 5.2 8.2 9.9 8.2h72.8c5.6 0 10.1-4.6 10.1-10.1v-1c-0.7-6.8-1.3-12.1-2-16-7.3-43.5-28-81.7-59.7-110.3-44.4-40.5-109.7-61.8-188.7-61.8-72.3 0-137.4 18.1-183.3 50.9-25.6 18.4-45.4 41.2-58.6 67.7-13.5 27.1-20.3 58.4-20.3 92.9 0 29.5 5.7 54.5 17.3 76.5 8.3 15.7 19.6 29.5 34.1 42H72c-4.4 0-8 3.6-8 8v60c0 4.4 3.6 8 8 8h433.2c2.1 0.4 3.9 0.8 5.9 1.2 30.9 6.2 49.5 10.4 66.6 15.2 23 6.5 40.6 13.3 55.2 21.5 35.8 20.2 53.3 49.2 53.3 89 0 35.3-15.5 66.8-43.6 88.8-30.5 23.9-75.6 36.4-130.5 36.4-43.7 0-80.7-8.5-110.2-25-29.1-16.3-49.1-39.8-59.7-69.5-0.8-2.2-1.7-5.2-2.7-9-1.2-4.4-5.3-7.5-9.7-7.5h-79.7c-5.6 0-10.1 4.6-10.1 10.1v1c0.2 2.3 0.4 4.2 0.6 5.7 6.5 48.8 30.3 88.8 70.7 118.8 47.1 34.8 113.4 53.2 191.8 53.2 84.2 0 154.8-19.8 204.2-57.3 25-18.9 44.2-42.2 57.1-69 13-27.1 19.7-57.9 19.7-91.5 0-31.8-5.8-58.4-17.8-81.4-5.8-11.2-13.1-21.5-21.8-30.8H952c4.4 0 8-3.6 8-8v-60c0-4.3-3.6-7.9-8-7.9z"
            /></svg
          >
        </button>
        <button
          on:click={() => clickFire("u")}
          class={`border-0  p-1 ${
            currentEditStatus.includes("u")
              ? "bg-gray-400 fill-white hover:bg-gray-500"
              : "hover:bg-gray-400 fill-sky-400 hover:fill-white "
          } rounded-md`}
        >
          <svg
            class="h-5 w-5"
            viewBox="0 0 1024 1024"
            version="1.1"
            xmlns="http://www.w3.org/2000/svg"
            width="200"
            height="200"
            ><path
              d="M341.333333 128v384a170.666667 170.666667 0 1 0 341.333334 0V128h85.333333v384a256 256 0 1 1-512 0V128h85.333333zM170.666667 853.333333h682.666666v85.333334H170.666667v-85.333334z"
            /></svg
          >
        </button>
      </div>
    </Float>
  {/if}
</div>

<style>
  hr {
    margin: 1rem;
    border: none;
    border-top: 0.5px solid rgb(204, 204, 204);
  }
</style>
