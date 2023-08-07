import { NextResponse } from "next/server";
import { connectDB } from "@/utils/mongoose";
import task from "@/models/task";
// TODO Piensa en como no depender de next para el api. me lo debo, tambien re-orientar otras apis creadas por mi.

export async function GET(request, { params }) {
	const method = "GET"
	try {
		connectDB()
		const taskFound = await task.findById(params.id)
		// FIXME Porque puta mierda no sirves?!
		if (taskFound === null) {
			return NextResponse.json({
				method,
				mensaje: "Tarea no encontrada",
				taskFound,
				debbug: [request, params]
			}, {
				status: 404
			})
		}
		return NextResponse.json({
			method,
			mensaje: `geting task... ${params.id}`,
			taskFound,
			debbug: [params, request]
		})
	} catch (error) {
		return NextResponse.json({
			method,
			mensaje: "Error al Buscar",
			message: error.message,
			debbug: [params, request],
			error
		}, {
			status: 400
		})
	}
}
export async function PUT(request, { params }) {
	const method = "PUT"
	try {
		connectDB()
		const data = await request.json()
		const taskUpdated = await task.findByIdAndUpdate(params.id, data, {
			new: true
		})
		if (taskUpdated === null) {
			return NextResponse.json({
				mensaje: "Tarea no encontrada",
				method,
				debbug: [data, request, params]
			}, {
				status: 404
			})
		}
		return NextResponse.json({
			mensaje: `geting task for update... ${params.id}`,
			method,
			taskUpdated,
			debbug: [data, request, params]
		})
	} catch (error) {
		return NextResponse.json({
			mensaje: "No se pudo actualizar",
			method,
			message: error.message,
			error,
			debbug: [request, params]
		}, {
			status: 400
		})
	}
}
export async function DELETE(request, { params }) {
	const method = "DELETE"
	try {
		connectDB()

		const taskDeleted = await task.findByIdAndDelete(params.id)
		if (taskDeleted === null) {
			return NextResponse.json({
				mensaje: "Tarea no encontrada",
				method,
				debbug: [request, params]
			}, {
				status: 404
			})
		}
		return NextResponse.json({
			mensaje: `Deleted... ${params.id}`,
			method,
			taskDeleted,
			debbug: [request, params]
		})
	} catch (error) {
		return NextResponse.json({
			mensaje: "No se pudo borrar",
			method,
			message: error.message,
			error,
			debbug: [request, params]
		}, {
			status: 400
		})
	}
}