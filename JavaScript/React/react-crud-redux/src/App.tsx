import "./App.css";
import { CreateContact } from "./components/CreateContact";
import { ListContacts } from "./components/ListContacs";

function App() {
	return (
		<>
			<ListContacts />
			<CreateContact />
		</>
	);
}

export default App;
