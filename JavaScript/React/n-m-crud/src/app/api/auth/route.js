import { NextResponse } from "next/server";
// TODO Piensa en como no depender de next para el api. me lo debo, tambien re-orientar otras apis creadas por mi.
export function GET() {
	return NextResponse.json({
		mensaje: "Hola mas facil api del mundo"
	})
}