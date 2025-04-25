extends Camera2D

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _input(event):
	if event.is_action("zoom_in"):
		zoom.x += 0.1;
		zoom.y += 0.1;
	elif event.is_action("zoom_out"):
		zoom.x -= 0.1;
		zoom.y -= 0.1;
