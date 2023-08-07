// option a
// inportar link para redireccionar
import Link from "next/link";
import styles from "./style.module.css"
function TaskCard({ task }) {
	return (
		<Link href={`/task/${task._id}`}>
			<div className={styles.fuck_life}>
				<h3 className={styles.fuck_life_title}>{task.title}</h3>
				<p className={styles.fuck_life_description}>{task.body}</p>
				<p className={styles.fuck_life_date}>
					<span className={styles.fuck_life_date_span}>
						Createad At:
					</span>
					{new Date(task.createdAt).toLocaleDateString()}
				</p>
			</div>
		</Link>
	)
}
export default TaskCard