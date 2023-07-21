// Import the functions you need from the SDKs you need
import { initializeApp } from "firebase/app";
import { getAuth } from "firebase/auth";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
const config = useRuntimeConfig()

const firebaseConfig = {
	apiKey: config.public.apiKey,
	authDomain: config.public.authDomain,
	projectId: config.public.projectId,
	storageBucket: config.public.storageBucket,
	messagingSenderId: config.public.messagingSenderId,
	appId: config.public.appId,
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
export const auth = getAuth(app);

export default app;
