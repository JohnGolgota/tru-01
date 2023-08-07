"use client"
import { ChangeEvent, FormEvent, useState, useEffect } from "react";
import { useRouter, useParams } from "next/navigation";
import styles from "./new.module.css";

function FormPage() {
	const [newTask, setNewTask] = useState({
		title: "",
		body: ""
	})
	const router = useRouter()
	const params = useParams()
	const getTask = async () => {
		const res = await fetch(`/api/task/${params.id}`, {
			method: "GET"
		})
		const data = await res.json()
		console.log("🚀 ~ file: page.tsx:18 ~ getTask ~ data:", data)

		setNewTask({
			title: data.taskFound.title,
			body: data.taskFound.body
		})

	}
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
				router.refresh()
			}
		} catch (error) {
			console.log(error);
		}
	}
	const handleSubmit = async (e: FormEvent) => {
		e.preventDefault()
		if (!params.id) {
			creatTask(newTask)
		} else {
			updateTask()
		}
		// Si no se actualizara el layout al redireccionar
		// router.refresh()
	}
	const handleChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
		setNewTask({
			...newTask,
			[e.target.name]: e.target.value
		})
	}
	const updateTask = async () => {
		try {
			const res = await fetch(`/api/task/${params.id}`, {
				method: "PUT",
				body: JSON.stringify(newTask),
				headers: {
					"Content-Type": "application/json"
				}
			})
			const data = await res.json()
			if (res.status === 200) {
				router.push('/')
				router.refresh()
			}
		} catch (error) {
			console.log(error);
		}
	}
	const handleDelete = async () => {
		try {
			if (window.confirm("Are u sure?")) {
				const res = await fetch(`/api/task/${params.id}`, {
					method: "DELETE"
				})
				router.push('/')
				router.refresh()
			}
		} catch (error) {
			console.log(error);

		}
	}
	useEffect(() => {
		params
		console.log("🚀 ~ file: page.tsx:44 ~ useEffect ~ params:", params)
		getTask()
	}, [])
	return (
		<div className={styles.sub_form}>
			<form onSubmit={handleSubmit}>
				<header className={styles.sub_header}>
					<h1 className={styles.sub_title}>
						{!params.id ? "Create Task" : "Update Task"}
					</h1>
					<div>
						<button className={styles.sub_delete} onClick={handleDelete} type="button">Delete</button>
					</div>
				</header>
				<input className={styles.sub_input} onChange={handleChange} value={newTask.title} type="text" name="title" placeholder="Title" />
				<textarea className={styles.sub_input} onChange={handleChange} value={newTask.body} name="body" id="bodyReq" placeholder="Description" rows={3}></textarea>
				<button className={styles.sub_btn} type="submit">
					{!params.id ? "Create" : "Update"}
				</button>
			</form>
		</div>
	)
}
export default FormPage