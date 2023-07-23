# Esta monda es utilizable desde un HTML
import pandas as pd
import mysql.connector
import sys

if len(sys.argv) < 2:
    print("Usege: python script '<args_1>'")
    sys.exit()

arg1 = sys.argv[1]
# print("Argument 1:", arg1)
def connect_to_database():
    return mysql.connector.connect(
        host='localhost',      # Reemplaza con la dirección de tu servidor MySQL
        user='root',   # Reemplaza con el nombre de usuario de la base de datos
        password='',   # Reemplaza con la contraseña del usuario
        database='import_test'  # Reemplaza con el nombre de la base de datos que deseas utilizar
    )

df = pd.read_csv(arg1) 

existing_documents = set()
# with open(arg1) as archivo:
#     contenido = archivo.read()
try:
    connection = connect_to_database()
    cursor = connection.cursor()

    # Obtener los números de documento que ya existen en la base de datos
    query = "SELECT Numero_Documento FROM `import_try`"  # Reemplaza 'nombre_de_la_tabla' con el nombre de tu tabla
    cursor.execute(query)
    fetch_a = cursor.fetchall()
    for doc in fetch_a:
        print("<br>fechas repetidas", doc)
        existing_documents.add(doc[0])

except mysql.connector.Error as error:
    print("Error al conectarse a la base de datos:", error)
    sys.exit()

finally:
    if connection.is_connected():
        cursor.close()
        connection.close()


# print("Contenido del arg", contenido)
new_records = df[~df['Número de Documento'].isin(existing_documents)]

# for arg in arg1:
#     print("Argument 1:", arg)
try:
    connection = connect_to_database()
    cursor = connection.cursor()

    for _, row in new_records.iterrows():
        # Supongamos que 'nombre_de_la_tabla' es el nombre de la tabla en la que deseas insertar los registros
        # Asegúrate de reemplazar los nombres de columna según corresponda
        query = f"""INSERT INTO import_try (Fecha, Reportar_a, Tipo_Documento, Numero_Documento,
                    Nombres, Telefono, Email, Direccion, Categoria, Descripcion, Cargar_Imagenes)
                    VALUES ('{row['Hora de envío']}', '{row['Reportar a']}', '{row['Tipo Documento']}',
                    {row['Número de Documento']}, '{row['Nombres y Apellidos']}', '{row['Teléfono']}',
                    '{row['Email']}', '{row['Dirección del Inmueble']}', '{row['Categoría']}',
                    '{row['Descripción']}', '{row['Cargar Imágenes']}')"""
        cursor.execute(query)

    connection.commit()

except mysql.connector.Error as error:
    print("Error al insertar en la base de datos:", error)
    sys.exit()

finally:
    if connection.is_connected():
        cursor.close()
        connection.close()

print("<br>Import completado.")