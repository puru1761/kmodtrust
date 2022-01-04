/*
 *
 * $Id$
 *
 * Copyright (c) 2022, Purushottam A. Kulkarni
 * All rights reserved.
 *
 */

#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/init.h>

static int __init hello_init(void)
{
	printk(KERN_INFO "Hello World!\n");

	return 0;
}

static void __exit hello_exit(void)
{
	printk(KERN_INFO "Goodbye World!\n");
}

module_init(hello_init);
module_exit(hello_exit);

MODULE_LICENSE("GPL");

MODULE_AUTHOR("Purushottam A. Kulkarni <puruk@protonmail.com>");
MODULE_DESCRIPTION("A hello world module");

