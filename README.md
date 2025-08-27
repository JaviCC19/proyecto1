# Raycaster Project

## Descripción
Este proyecto es un **raycaster 3D** implementado en Rust usando la librería [raylib](https://www.raylib.com/).  
El jugador puede moverse por laberintos, recolectar sprites (*perites*), visualizar un minimapa y pasar niveles, todo mientras se reproduce música de fondo.  



## Características implementadas

### Movilidad y cámara
- Movimiento hacia adelante y atrás.  
- Rotación con teclado.  
- **Rotación horizontal con el mouse** implementada.  
- Soporte para **control de Play / gamepad** implementado.  
- FPS desplegados y mantenidos alrededor de **15 fps**.

### Interfaz y visual
- Minimap en la esquina superior izquierda que muestra la posición del jugador y sprites restantes.  
- Animaciones en sprites al recolectarlas (desaparecen con efecto visual).  
- Estética de niveles cuidada, con texturas para paredes y sprites.  
- Pantalla de bienvenida con **selección de múltiples niveles**.  
- Pantalla de éxito cuando el jugador recolecta todos los sprites del nivel.  

### Sonido
- Música de fondo reproducida durante el juego (**Taylor Swift opcional si se reemplaza la música actual**).  
- Sonido de efectos al interactuar con sprites (colección).

### Controles
- Teclas `W`, `S` para avanzar/retroceder.  
- Teclas `A`, `D` para girar.  
- Rotación horizontal con mouse.  
- Soporte para control de Play: ejes y botones mapeados para movimiento y rotación.  
- Tecla `ESC` para salir al menú principal.  




## Instrucciones para correr
1. Clonar el repositorio.  
2. Instalar Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)  
3. Instalar [raylib](https://www.raylib.com/) y dependencias (según tu sistema operativo).  
4. Ejecutar:
   ```bash
   cargo run --release

