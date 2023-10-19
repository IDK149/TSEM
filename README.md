# TSEM
Este proyecto nace con el objetivo de brindar flexibilidad a la hora de escribir partituras, donde se busca rapidez y eficiencia.

## Requisitos
- Rust
- Lassus Font V2 (Una alteración de la font original "Lassus")

## Instalación
Clonar el repositorio teniendo en cuenta los requerimientos del proyecto.

## Comenzar a escribir:
Luego de clonar el proyecto, dentro de la carpeta src, crear un archivo llamado input.txt
### Codigo de prueba
```
psize(200, 200)
fsize(100)
title(Mi primera partitura)

clave(F)[
	(4/4){
		nota(4A)
		nota(4C)
		nota(2Bb)
	}
	(3/4){
		nota(2Ab)
		nota(4C)
	}
	(4/4){
		nota(1B#)
	}
]
```
Al ejecutar se creará un archivo .pdf con el formato deseado.

> **Nota**: la indentación no es importante, sin embargo el uso de corchetes, paréntesis y llaves es de uso obligatorio.


## Futuras mejoras:
| Nombre                            | Descripción         |
|----------------------------------|---------------------|
| Comando macro                     |El usuario tendrá el comando "macro()" y aqui podrá poner sus propios atajos que serán configuradores en un archivo .toml externo y aun en desarrollo.|
| Configuración de páginas          | Agregar comandos para el estilo de la pagina: Margenes, tamaño de la fuente secundaria, distancia entre lineas.|
| Optimización de código            | Buscar alternativas en la creación de código para reducir el tiempo de compilación.|
| Manejo de Octavas en las escalas  | Solo está definido una nota por clave, no existen para el programa dos notas en una clave.|
| Corcheas, semicorcheas            | No se pueden escribir corcheas ni semicorcheas.|
| Salto de línea inteligente        | Cuando se llegue al límite de la página debe hacer un salto sin afectar la lectura de la partitura.|
|Gestión de código|Manejo de errores y modularidad en el código para hacer más facil su lectura y edición|

> **Nota**: Este proyecto no sería posible sin la font de Lassus, no obstante, esta font no contiene la cantidad de caracteres necesarios para el proyecto, por lo cual estaré limitado a ella en la medida que la modifique y genere esos nuevos símbolos para la comodidad en la escritura del usuario.
