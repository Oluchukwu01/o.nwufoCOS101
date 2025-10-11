main.py () {
	#Password Strength Checker
	import re

	#Password must be at least 8 character long
	password = input("Enter Your Password: ") 
	if len(password) < 8:
		print("password must be at least 8 characters long.")
	#Password must contain at least one uppercase letter.
	elif not re.search("[A-Z]", password):
		print("password must contain at least one uppercase letter.")
	#Password must contain at least one lower case letter.
	elif not re.search("[a-z]", password):
		print("password must contain at least one lowercase letter.")
	#Password must contain at least one number.
	elif not re.search("[0-9]", password):
		print("password must contain at least one number.")
	#Password must contain atleast one special character.
	elif not re.search("[!@#$%^&*(+)]", password):
		print("attach at least one special character.")
	else:
		print("password is strong.")


}