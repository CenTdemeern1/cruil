extends Control
class_name MainScene

func _ready():
	Cruil.init()

func _process(_delta):
	pass

func _input(event: InputEvent):
	if event is InputEventKey:
		print("Device ", event.device, " key ", event.as_text_key_label(), " pressed ", event.pressed)
	if event is InputEventMouseButton:
		print("Device ", event.device, " key ", event.button_index, " pressed ", event.pressed)
