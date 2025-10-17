using System.Reflection;
using System.Runtime.InteropServices;

namespace faml;

public static class FFI
{
    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_expr_from_str([MarshalAs(UnmanagedType.LPStr)] string psrc, out IntPtr ppexpr, out IntPtr pperr);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_expr_set_none(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_expr_set_bool(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, int value);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_expr_set_int(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, long value);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_expr_set_float(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, double value);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_expr_set_string(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, [MarshalAs(UnmanagedType.LPStr)] string pvalue);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_expr_evalute(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, out IntPtr ppval, out IntPtr pperr);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_is_none(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_is_bool(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_as_bool(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_is_int(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern long faml_value_as_int(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_is_float(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern double faml_value_as_float(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_is_str(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr faml_value_as_str(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_is_array(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_get_array_length(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_is_map(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_get_map_length(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr faml_value_get_keys(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_set_none(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_value_set_bool(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath, int value);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_value_set_int(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath, long value);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_value_set_float(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath, double value);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int faml_value_set_string(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath, [MarshalAs(UnmanagedType.LPStr)] string pvalue);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_release_expr(IntPtr pexpr);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_release_value(IntPtr pval);

    [DllImport("faml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void faml_release_str(IntPtr pstr);

    static FFI()
    {
        IntPtr DllImportResolver(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
        {
            if (libraryName == "faml.dll")
            {
                if (Environment.Is64BitProcess)
                {
                    if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                    {
                        return NativeLibrary.Load("libs/faml.x86_64_win.dll", assembly, searchPath);
                    }
                    else if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
                    {
                        return NativeLibrary.Load("libs/libfaml.x86_64_linux.so", assembly, searchPath);
                    }
                }
                throw new Exception("unsupported platform");
            }
            return IntPtr.Zero;
        }
        NativeLibrary.SetDllImportResolver(Assembly.GetExecutingAssembly(), DllImportResolver);
    }
}