import Styles from "./styles.module.css";
import Link from 'next/link'
function Navbar() {
	return (
		<nav className={Styles.navbar}>
			<div className={Styles.navbar_conteiner}>
				<div>
					<Link href="/">
						<h1 className={Styles.navbar_home}>
							Home
						</h1>
					</Link>
				</div>
				<ul className={Styles.navbar_ul}>
					<li>
						<Link href="/task/new">New Task</Link>
					</li>
				</ul>
			</div>
		</nav>
	)
}
export default Navbar