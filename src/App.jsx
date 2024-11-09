import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
	const [textareaVal, setTextareaVal] = useState("");

	async function load_file() {
		let data = await invoke("open_file", {});
		console.log(data)
		setTextareaVal(data);
	}

	async function save_file() {
		await invoke("save_file", {content: textareaVal});
	}

	return (
		<main className="container">
		<h1>Text Editor</h1>

		<div className="row">
		</div>
		<p></p>

		<form
			className="row"
			onSubmit={(e) => {
			e.preventDefault();
			load_file();
			}}
		>
			<button type="submit">Load</button>
		</form>

		<p></p>

		<form
			className="row"
			onSubmit={(e) => {
			e.preventDefault();
			save_file();
			}}
		>
			<button type="submit">Save</button>
		</form>

		<p></p>

		<textarea
			style={{ width: `calc(100vw - 50px)`, height: `calc(100vh - 50px)` }}
			className='row'
			value = {textareaVal}
			onChange = {(e)=>{
						setTextareaVal(e.target.value
						);
					}}
		>
		</textarea>
		</main>
	);
}

export default App;
