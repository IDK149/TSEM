# TSEM
## ¿Qué es TSEM?
TSEM por el momento es un CLI(_command-line interface_) que transforma un archivo de texto (.txt), en partituras musicales, basado en una [sintaxis](./docs/syntax.md) sencilla y eficiente.

## Requisitos
- Lassus Font V2 (Una alteración de la font original "Lassus")
- DejaVuSansMNerdFont-Bold.tff

## Instalación
Dentro de los [realeases](https://github.com/IDK149/TSEM/releases/) del proyecto, podrá encontrar los archivos ejecutables del proyecto, instale y coloque dentro de su ``PATH`` el archivo ejecutable para que esté disponible de manera local en su entorno.
> **Nota**: Por el momento solo se cuenta con archivos ejecutables para windows pero se está trabajando para demás sistemas operativos.

# Primeras notas
Luego de instalar el programa podrá escribir sus primeras notas. Para esto, cree un nuevo archivo .txt y escriba el código que visualiza en el **código de prueba**. Guarde el archivo y ejecute el programa desde terminal pasando como argumento la ruta del archivo .txt

### Código de prueba
```
title(Mi primera partitura)
sub(Grupo De la composición)
autor(El autor)

clave(G)[
	(4/4){
		nota(4A)
		nota(4C)
		nota(2Bb)
	}
	(3/4){
		nota(2Ab)
		nota(4C)
	}
]

```
Al ejecutar se creará un archivo .pdf con el formato deseado.

> **Nota**: la indentación no es importante, sin embargo, el uso de corchetes, paréntesis y llaves es de uso obligatorio.


## Futuras mejoras:
| Nombre                            | Descripción         |
|----------------------------------|---------------------|
| Comando macro                     |El usuario tendrá el comando "macro()" y aqui podrá poner sus propios atajos que serán configuradores en un archivo .toml externo|
| Configuración de páginas          | Agregar comandos para el estilo de la pagina: Margenes, tamaño de la fuente secundaria, distancia entre lineas.|
| Optimización de código            | Buscar alternativas en la creación de código para reducir el tiempo de compilación.|
| Manejo de Octavas en las escalas  | Solo está definido una nota por clave, no existen para el programa dos notas en una clave.|
| Corcheas, semicorcheas            | No se pueden escribir corcheas ni semicorcheas.|
| Salto de línea inteligente        | Cuando se llegue al límite de la página debe hacer un salto sin afectar la lectura de la partitura.|
|Gestión de código|Manejo de errores y modularidad en el código para hacer más fácil su lectura y edición|

## Proyectos similares
[musicXML](https://www.musicxml.com/)

> **Nota**: Este proyecto no sería posible sin la font de Lassus, no obstante, esta font no contiene la cantidad de caracteres necesarios, por lo cual estaré limitado a ella en la medida que la modifique y genere nuevos símbolos para la comodidad en la escritura del usuario, si deseas ayudar, contribuyendo a la creación de nuevos caracteres o la corrección de los mismos, estoy más que dispuesto a escuchar y atender a las peticiones.
