USE [WTPV_CALEIA_TEST];
GO
SET IDENTITY_INSERT [dbo].[TP_ARTICULO] ON
GO
INSERT [dbo].[TP_ARTICULO] ([Articulo], [Nombre], [Familia], [Barras], [Combinable], [PorcentajeCombina], [Comentario], [Logo], [Fecha_Modificado], [Usuario], [Codigoid])
VALUES
(CAST(1 AS Numeric(9, 0)), N'2x1 vino casa', N'0104', N'', 0, CAST(100.00 AS Numeric(7, 2)), 0, 0x, CAST(N'2021-08-31T18:28:44.290' AS DateTime), N'MASTER', CAST(6435 AS Numeric(18, 0)))
GO
SET IDENTITY_INSERT [dbo].[TP_ARTICULO] OFF
GO
