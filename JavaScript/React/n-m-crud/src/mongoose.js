import { connect, connection, connections } from 'mongoose';
const conn = {
	isConnected: false
}
export async function connectDB() {
	if (conn.isConnected) return;
	const db = await connect("mongodb://localhost:27017/nextmongocrud")
	conn.isConnected = connections[0].readyState
}
connection.on('connection', () => {
	console.log('someone connected!');
});
connection.on('error', () => {
	console.log('someone NO connected!');
});