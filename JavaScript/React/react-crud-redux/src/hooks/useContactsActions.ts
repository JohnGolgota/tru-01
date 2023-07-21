import {
	UserId,
	addNewContact,
	contact,
	deleteContactById,
} from "../store/contacts/slice";
import { useAppDispatch } from "./store";

export const useContactsActions = () => {
	const dispatch = useAppDispatch();

	const addContact = (newContact: contact) => {
		dispatch(addNewContact(newContact));
	};
	const RemoveContact = (id: UserId) => {
		dispatch(deleteContactById(id));
	};
	return { addContact, RemoveContact };
};
