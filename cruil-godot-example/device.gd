extends Control
class_name DeviceScene

@export var device: int = 0
var keyboard_tween: Tween
var mouse_move_tween: Tween
var mouse_button_tween: Tween

func _input(event: InputEvent):
	if event.device != device:
		return
	if event is InputEventKey:
		%Mouse.hide()
		%Keyboard.show()
		if event.echo:
			return
		if keyboard_tween:
			keyboard_tween.kill()
		if event.pressed:
			%Keyboard.position.x = randf() * 40 - 20
			%Keyboard.position.y = randf() * 20
			%Keyboard.rotation_degrees = randf() * 30 - 15
		else:
			keyboard_tween = create_tween().set_ease(Tween.EASE_IN).set_trans(Tween.TRANS_EXPO)
			keyboard_tween.parallel().tween_property(%Keyboard, "position", Vector2.ZERO, 0.5)
			keyboard_tween.parallel().tween_property(%Keyboard, "rotation", 0, 0.5)
	if event is InputEventMouse:
		%Keyboard.hide()
		%Mouse.show()
		if event is InputEventMouseMotion:
			if mouse_move_tween:
				mouse_move_tween.kill()
			%Mouse.position += event.relative
			mouse_move_tween = create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_CIRC)
			mouse_move_tween.tween_property(%Mouse, "position", Vector2.ZERO, 0.5)
		if event is InputEventMouseButton:
			if mouse_button_tween:
				mouse_button_tween.kill()
			if event.pressed:
				%Mouse.scale = Vector2(0.8, 0.8)
			else:
				mouse_button_tween = create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_CIRC)
				mouse_button_tween.tween_property(%Mouse, "scale", Vector2.ONE, 0.5)
