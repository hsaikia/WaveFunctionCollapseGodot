extends WfcMap

@export var map_size : Vector2i = Vector2i(10, 10)

func _input(event):
	if event.is_action_released("generate"):
		generate_new(map_size)
	if event.is_action_pressed("quit"):
		get_tree().quit()
