
const APP = document.getElementById('app')
const list = document.getElementById('body_res')
const API = ".env"

async function getData(url = '') {
    const response = await fetch(url, {
        method: 'GET',
    });
    return response.json();
}

const template_tr = objeto => {
    return `
	<td class="responsive-active"><img src="assets/images/svg/plus-circle-fill.svg" alt="toggle button"> </td>

	<td>${objeto.id_pqrs}</td>
	
	<td class="responsive-hidden">${objeto.fecha_registro}</td>

	<td>${objeto.identificacion}</td>

	<td>${objeto.nombres_apellidos}</td>
	
	<td class="responsive-hidden">${objeto.tel}</td>

	<td><span class="st-pending">${objeto.estado}</span></td>

	<td class="responsive-hidden">
		<form method="POST" id="delete${objeto.id_pqrs}">
			<input type="hidden" value="${objeto.id_pqrs}" name="eliminarPqrs" id="btnDelete">
			<button type="button" class="btn btn-primary btnEliminar">&nbsp;ELIMINAR</button>
		</form>
	</td>
`}
/**
 * 
 * @param {object} objeto registro pqrs ejemplo
 * @returns HTMLTableRowElement
 */
const createElementPqrs = objeto => {
    const pqrs = document.createElement('tr')
    pqrs.classList.add('even')
    pqrs.innerHTML = template_tr(objeto)
    return pqrs;
}

async function ListarPQRS() {
    list.innerHTML = ''
    getData(API).then(res => {
        res.forEach(pqrs => {
            pqrs_td = createElementPqrs(pqrs)
            list.appendChild(pqrs_td)
        });
    })
}