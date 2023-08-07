import { connectDB } from "@/utils/mongoose";
import Task from "@/models/task";
import TaskCard from "@/components/TaskCard/TaskCard";
import StylesPage from "@/app/StylesModules/page.module.css"
async function loadTask() {
	connectDB()
	const tasks = await Task.find()
	return tasks
}
async function HomePage() {
	const tasks = await loadTask()
	return (
		<div>
			<div className={StylesPage.page}>
				{tasks.map(task => (
					<TaskCard task={task} key={task._id} />
				))}
			</div>
		</div >
	)
}
export default HomePage