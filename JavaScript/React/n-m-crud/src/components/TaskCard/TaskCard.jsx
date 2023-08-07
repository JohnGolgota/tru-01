// option a
// inportar link para redireccionar
import Link from "next/link";
import styles from "./style.module.css"
function TaskCard({ task }) {
	return (
		<Link href={`/task/${task._id}`}>
			<div className={styles.fuck_life}>
				<h1>{task.title}</h1>
				<p>{task.body}</p>
			</div>
		</Link>
	)
}
export default TaskCard