# Mini OBJ Mesh
A simplified representation of 3D mesh data structures with code generation.

## Introduction
The mini obj data structure is a data structure representing 3D objects. The purpose
of it is to have a simplified representation of 3D meshes for games and computer graphics.
In particular, the object mesh structure in this library only represents vertices, normals, 
and texture coordinates. One feature this library also has is code generation for embedding 
object meshes into a binary at compile time. This cuts down on the amount of work the program
needs to do at runtime for loading art assets in small games and graphics demos, since one embeds 
the parsed data structure directly into the source code.

## Usage
