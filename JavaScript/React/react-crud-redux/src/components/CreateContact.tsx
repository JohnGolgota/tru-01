import { Badge, Button, Card, TextInput, Title } from "@tremor/react";
import { useState } from "react";
import { useContactsActions } from "../hooks/useContactsActions";
import { contact } from "../store/contacts/slice";

export function CreateContact() {
	const { addContact } = useContactsActions();
	const [result, setResult] = useState<"ok" | "oh, oh" | null>();
	const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
		event.preventDefault();
		setResult(null);
		const form = event.target as HTMLFormElement;
		const formData = new FormData(form);

		const contact: contact = {
			name: formData.get("name") as string,
			email: formData.get("email") as string,
			github: formData.get("github") as string,
		};
		if (!contact.email || !contact.github || !contact.name) {
			return setResult("oh, oh");
		}
		addContact(contact);
		setResult("ok");
		form.reset();
	};
	return (
		<Card style={{ marginTop: "16px" }}>
			<Title>Create New Contact</Title>
			<form onSubmit={handleSubmit}>
				<TextInput name="name" placeholder="Nombre" />
				<TextInput name="email" placeholder="Email" />
				<TextInput name="github" placeholder="Github" />
				<div>
					<Button type="submit" style={{ marginTop: "16px" }}>
						Crear Usuario
					</Button>
					<span>
						{result === "ok" && (
							<Badge color="green">Guardado correctamente</Badge>
						)}
						{result === "oh, oh" && (
							<Badge color="red">Error con los campos</Badge>
						)}
					</span>
				</div>
			</form>
		</Card>
	);
}
