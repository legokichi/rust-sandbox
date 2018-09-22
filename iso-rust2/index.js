window.document.addEventListener("DOMContentLoaded", async ()=>{
	const { main } = await import("./js_hello_world");
	main();
});