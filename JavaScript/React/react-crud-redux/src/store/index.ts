import { configureStore, type Middleware } from "@reduxjs/toolkit";
import { toast } from "sonner";
import {
	contactsSlice,
	contactsWithId,
	rollBackDeleteContact,
	UserId,
} from "./contacts/slice.ts";

const persistanceLocalStorageMiddleware: Middleware =
	(store) => (next) => (action) => {
		next(action);
		localStorage.setItem("__redux__state__", JSON.stringify(store.getState()));
	};
const syncWithDataBase: Middleware = (store) => (next) => (action) => {
	const { type, payload } = action;

	const previusState = store.getState();
	next(action);
	/**
	 * Este funcionamiento estaria separado.
	 */
	if (type === "contacts/deleteContactById") {
		const contactIdTiRemove: UserId = payload;
		const ContactToRemove: contactsWithId = previusState.contacts.find(
			(contact: contactsWithId) => contact.id === contactIdTiRemove,
		);
		fetch(`https://jsonplaceholder.typicode.com/users${contactIdTiRemove}`, {
			method: "DELETE",
		})
			.then((res) => {
				if (res.ok) {
					toast.success(`Usuario ${contactIdTiRemove} Fue eliminado.`);
				}
				throw new Error("Error al eliminar contacto");
			})
			.catch((err) => {
				toast.error(`Error deleting contact ${contactIdTiRemove}`);
				if (ContactToRemove) {
					store.dispatch(rollBackDeleteContact(ContactToRemove));
				}
				console.error("Valio verga", err);
			});
	}
};
export const store = configureStore({
	reducer: {
		contacts: contactsSlice.reducer,
	},
	middleware: [persistanceLocalStorageMiddleware, syncWithDataBase],
});
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
