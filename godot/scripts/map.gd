extends WfcMapLayer

@onready var cam: Camera2D = $Cam
@export var generation_interval : float = 2.0
var time_elapsed : float = 0.0

func _process(delta: float) -> void:
	if time_elapsed < generation_interval:
		time_elapsed += delta;
		return
	time_elapsed = 0.0
	cam.position.x = 132 / 2.0;
	cam.position.y = (map_size.y / 2.0) * 66.0 
	generate_new()
	

func _input(event):
	if event.is_action_released("generate"):
		generate_new()
	if event.is_action_pressed("quit"):
		get_tree().quit()
