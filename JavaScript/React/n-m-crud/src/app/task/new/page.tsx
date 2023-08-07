"use client"
import { ChangeEvent, FormEvent, TextareaHTMLAttributes, useState } from "react";
import { useRouter } from "next/navigation";
import styles from "./new.module.css";

function FormPage() {
	const [newTask, setNewTask] = useState({
		title: "",
		body: ""
	})
	const router = useRouter()
	const creatTask = async (task: object) => {
		try {
			const res = await fetch('/api/task', {
				method: "POST",
				body: JSON.stringify(task),
				headers: {
					"Content-Type": "application/json"
				}
			})
			const data = await res.json()
			if (res.status === 200) {
				router.push('/')
			}
		} catch (error) {
			console.log(error);
		}
	}
	const handleSubmit = async (e: FormEvent) => {
		e.preventDefault()
		const res = await creatTask(newTask)
		// Si no se actualizara el layout al redireccionar
		// router.refresh()
	}
	const handleChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
		setNewTask({
			...newTask,
			[e.target.name]: e.target.value
		})
	}
	return (
		<div className={styles.sub_form}>
			<form onSubmit={handleSubmit}>
				<h1 className={styles.sub_title}>Create Task</h1>
				<input className={styles.sub_input} onChange={handleChange} type="text" name="title" placeholder="Title" />
				<textarea className={styles.sub_input} onChange={handleChange} name="body" id="bodyReq" placeholder="Description" rows={3}></textarea>
				<button className={styles.sub_btn}>Save</button>
			</form>
		</div>
	)
}
export default FormPage