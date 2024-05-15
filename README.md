# what is rmgl
rmgl is for **ray marching graphics library**
it is a graphics library for ray marching using opengl and rust  
demo and modeling at //link here
almost every thing write with the help of https://iquilezles.org/articles/ so check him out 
## License
it is free and open source under the MIT License (http://opensource.org/licenses/MIT)
I am not a lawyer and will be very happy and surprised if someone do any thing with that code 


# modeling
modeling is use an sdf file
sdf file look like this
![enter image description here](https://cdn.discordapp.com/attachments/771848532637384704/1240342702436913152/ok.jpg?ex=664636a8&is=6644e528&hm=51ccc3f2c5da0539b3ec29efd29ea49b54cb7b528288c0ac954c5b9513ce3b19&)
you can edit a sdf file washout learn glsl using 
## SDFMAKER  
go to project demo 
click on modeling
### controls 
caps - change from free camera to not free camera  and reverse
**free camera**
move - wasd and mouse 
**not free camera**
rotate - wasd 
move -q and e
reset camera - cutrl+f

cutrl+r - save as sdf file
space - update to screen from object_maker.sdfMaker to screen
escape - return to main menu

to actually change object you need to go the object_maker.sdfMaker file on the demo //again link here
(one day will be shit with UI I promise)

### object_maker.sdfMaker
on sdfMaker to write boolean we write T for true and F false
the sdfMaker files start with the settings
name: the name of object 
max_rays: maximum steps each ray will do,
min_dis_ray: the distance of ray that is close enough to object so it counted,
max_dis_ray: camare max distance,
show_above_min_dis_errors: (boolean) if ray hit the maximum steps of ray will it is not beckgrund,
color_senstivity: how much distance effect color,
color_offset: color offset,
dis_from_zero:boolean(if false the distance effct color will be relative to camera and if true wiil be relative to (0,0,0))

colors: palate - 4 colors  in this format
 [(0.8, 0.5, 0.4),(0.2, 0.4, 0.2),(2.0, 1.0, 1.0),(0.0, 0.30, 0.30),],   
 background: for background u have 4 options
 1:color in this format  color(r,g,b) - example: background: color(0.1,0.1,0.1)
  2:image in this format image:(image_path)- example: background: image:(path_to_image.png)
  3: continuation of ray
  will act like there is no background and the ray hit something the first float is the color senstivity and the secand the color offset if color sensitivity is 0 and color offset is not zero so it have the same color senstivity and offset of the scene - in this format of cor(color sensitivity ,color offset ) -example cor(0.1,0.0)
