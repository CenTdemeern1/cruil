extends Control
class_name MainScene

const DEVICE_SCENE: PackedScene = preload("res://device.tscn")
var devices: Array[CruilDevice]

func _ready():
	if !Cruil.init():
		return
	devices = Cruil.open_all()
	var id = 0
	for device in devices:
		id += 1
		device.map_event_id(id)
		var device_scene: DeviceScene = DEVICE_SCENE.instantiate()
		device_scene.device = id
		$GridContainer.add_child(device_scene)

#func _process(_delta):
	#for device in devices:
		#var event = device.get_input_event()
		#if !event:
			#continue
		#print(event)
		#event.free()

#func _input(event: InputEvent):
	#if event.device == 0:
		#return
	#if event is InputEventKey:
		#print("Device ", event.device, " key ", event.as_text_key_label(), " pressed ", event.pressed)
	#if event is InputEventMouseButton:
		#print("Device ", event.device, " button ", event.button_index, " pressed ", event.pressed)
