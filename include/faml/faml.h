#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Try parse string and get faml-expr pointer
 */
int faml_expr_from_str(const char *psrc, void **ppexpr, const char **pperr);

void faml_expr_set_none(void *pexpr, const char *ppath);

void faml_expr_set_bool(void *pexpr, const char *ppath, int value);

void faml_expr_set_int(void *pexpr, const char *ppath, long long value);

void faml_expr_set_float(void *pexpr, const char *ppath, double value);

void faml_expr_set_string(void *pexpr, const char *ppath, const char *pvalue);

int faml_expr_evalute(void *pexpr, const char *ppath, void **ppval, const char **pperr);

int faml_value_is_none(void *pval, const char *ppath);

int faml_value_is_bool(void *pval, const char *ppath);

int faml_value_as_bool(void *pval, const char *ppath);

int faml_value_is_int(void *pval, const char *ppath);

long long faml_value_as_int(void *pval, const char *ppath);

int faml_value_is_float(void *pval, const char *ppath);

double faml_value_as_float(void *pval, const char *ppath);

int faml_value_is_str(void *pval, const char *ppath);

const char *faml_value_as_str(void *pval, const char *ppath);

int faml_value_is_array(void *pval, const char *ppath);

int faml_value_get_array_length(void *pval, const char *ppath);

int faml_value_is_map(void *pval, const char *ppath);

int faml_value_get_map_length(void *pval, const char *ppath);

const char *faml_value_get_keys(void *pval, const char *ppath);

int faml_value_set_none(void *pval, const char *ppath);

void faml_value_set_bool(void *pval, const char *ppath, int value);

void faml_value_set_int(void *pval, const char *ppath, long long value);

void faml_value_set_float(void *pval, const char *ppath, double value);

int faml_value_set_string(void *pval, const char *ppath, const char *pvalue);

void faml_release_expr(const void *pexpr);

void faml_release_value(const void *pval);

void faml_release_str(const char *pstr);
